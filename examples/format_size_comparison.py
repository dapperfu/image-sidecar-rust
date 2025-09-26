#!/usr/bin/env python3
"""
Format Size Comparison Demo for sportball-sidecar-rust

This demo compares the file sizes of different sidecar formats:
- JSON (human-readable, largest)
- Binary (bincode, medium)  
- Rkyv (zero-copy, smallest)
"""

import tempfile
import os
from sportball_sidecar_rust import SportballSidecar, OperationType, SidecarFormat


def create_test_data_with_different_sizes():
    """Create test data of varying complexity to show format differences."""
    return [
        {
            "name": "Simple Data",
            "data": {
                "face_detection": {
                    "success": True,
                    "faces": [{"bbox": [100, 100, 200, 200], "confidence": 0.95}]
                }
            }
        },
        {
            "name": "Medium Data", 
            "data": {
                "face_detection": {
                    "success": True,
                    "faces": [
                        {"bbox": [100, 100, 200, 200], "confidence": 0.95},
                        {"bbox": [300, 300, 150, 150], "confidence": 0.87},
                        {"bbox": [500, 200, 180, 180], "confidence": 0.92}
                    ],
                    "processing_time": 0.123,
                    "model_version": "v1.2.3",
                    "metadata": {
                        "timestamp": "2024-01-01T12:00:00Z",
                        "camera_id": "cam_001",
                        "location": "field_center"
                    }
                }
            }
        },
        {
            "name": "Complex Data",
            "data": {
                "face_detection": {
                    "success": True,
                    "faces": [
                        {
                            "bbox": [100, 100, 200, 200], 
                            "confidence": 0.95,
                            "landmarks": [
                                [120, 130], [180, 130], [150, 150], 
                                [130, 180], [170, 180]
                            ],
                            "emotions": {"happy": 0.8, "neutral": 0.2}
                        },
                        {
                            "bbox": [300, 300, 150, 150], 
                            "confidence": 0.87,
                            "landmarks": [
                                [320, 310], [380, 310], [350, 330],
                                [330, 360], [370, 360]
                            ],
                            "emotions": {"happy": 0.6, "neutral": 0.4}
                        },
                        {
                            "bbox": [500, 200, 180, 180], 
                            "confidence": 0.92,
                            "landmarks": [
                                [520, 210], [580, 210], [550, 230],
                                [530, 260], [570, 260]
                            ],
                            "emotions": {"happy": 0.9, "neutral": 0.1}
                        }
                    ],
                    "processing_time": 0.456,
                    "model_version": "v1.2.3",
                    "metadata": {
                        "timestamp": "2024-01-01T12:00:00Z",
                        "camera_id": "cam_001", 
                        "location": "field_center",
                        "weather": "sunny",
                        "temperature": 22.5,
                        "humidity": 65.0
                    },
                    "quality_metrics": {
                        "sharpness": 0.85,
                        "brightness": 0.78,
                        "contrast": 0.82,
                        "noise_level": 0.15
                    }
                }
            }
        },
        {
            "name": "Large Dataset",
            "data": {
                "face_detection": {
                    "success": True,
                    "faces": [
                        {
                            "bbox": [i*50, i*50, 100, 100], 
                            "confidence": 0.8 + (i % 20) * 0.01,
                            "landmarks": [[i*50+10, i*50+10], [i*50+90, i*50+10], [i*50+50, i*50+50]],
                            "emotions": {"happy": 0.7 + (i % 10) * 0.02, "neutral": 0.3 - (i % 10) * 0.02},
                            "attributes": {
                                "age": 25 + (i % 30),
                                "gender": "male" if i % 2 == 0 else "female",
                                "glasses": i % 3 == 0,
                                "smile": i % 4 == 0
                            }
                        }
                        for i in range(20)  # 20 faces
                    ],
                    "processing_time": 1.234,
                    "model_version": "v1.2.3",
                    "metadata": {
                        "timestamp": "2024-01-01T12:00:00Z",
                        "camera_id": "cam_001",
                        "location": "field_center",
                        "weather": "sunny",
                        "temperature": 22.5,
                        "humidity": 65.0,
                        "wind_speed": 5.2,
                        "pressure": 1013.25
                    },
                    "quality_metrics": {
                        "sharpness": 0.85,
                        "brightness": 0.78,
                        "contrast": 0.82,
                        "noise_level": 0.15,
                        "saturation": 0.88,
                        "hue": 0.45
                    },
                    "technical_info": {
                        "image_resolution": [1920, 1080],
                        "focal_length": 50.0,
                        "aperture": 2.8,
                        "iso": 400,
                        "shutter_speed": "1/125"
                    }
                }
            }
        }
    ]


def compare_format_sizes():
    """Compare file sizes across different formats."""
    test_cases = create_test_data_with_different_sizes()
    
    with tempfile.TemporaryDirectory() as temp_dir:
        print("📊 FORMAT SIZE COMPARISON")
        print("=" * 60)
        
        sidecar = SportballSidecar()
        
        for test_case in test_cases:
            print(f"\n--- {test_case['name']} ---")
            
            # Create image file
            image_path = os.path.join(temp_dir, f"{test_case['name'].lower().replace(' ', '_')}.jpg")
            with open(image_path, "wb") as f:
                f.write(b"fake image data " * 100)
            
            # Test each format
            formats = [
                (SidecarFormat.JSON, "JSON"),
                (SidecarFormat.BINARY, "Binary"), 
                (SidecarFormat.RKYV, "Rkyv")
            ]
            
            format_results = {}
            
            for fmt, fmt_name in formats:
                # Set format and create sidecar
                sidecar.set_default_format(fmt)
                result = sidecar.create_sidecar(image_path, OperationType.FACE_DETECTION, test_case['data'])
                
                # Get file size
                file_path = result['sidecar_path']
                file_size = os.path.getsize(file_path)
                format_results[fmt_name] = {
                    'size': file_size,
                    'path': file_path
                }
                
                print(f"{fmt_name:8}: {file_size:6} bytes")
            
            # Calculate compression ratios
            json_size = format_results['JSON']['size']
            binary_size = format_results['Binary']['size']
            rkyv_size = format_results['Rkyv']['size']
            
            print(f"\nCompression ratios (vs JSON):")
            print(f"Binary: {json_size/binary_size:.2f}x smaller ({((json_size-binary_size)/json_size)*100:.1f}% reduction)")
            print(f"Rkyv:   {json_size/rkyv_size:.2f}x smaller ({((json_size-rkyv_size)/json_size)*100:.1f}% reduction)")
            print(f"Rkyv vs Binary: {binary_size/rkyv_size:.2f}x smaller ({((binary_size-rkyv_size)/binary_size)*100:.1f}% reduction)")


def demonstrate_format_efficiency():
    """Demonstrate format efficiency with many files."""
    with tempfile.TemporaryDirectory() as temp_dir:
        print(f"\n🚀 FORMAT EFFICIENCY DEMO")
        print("=" * 60)
        
        sidecar = SportballSidecar()
        
        # Create 100 files in each format
        num_files = 100
        formats = [
            (SidecarFormat.JSON, "JSON"),
            (SidecarFormat.BINARY, "Binary"),
            (SidecarFormat.RKYV, "Rkyv")
        ]
        
        total_sizes = {}
        
        for fmt, fmt_name in formats:
            print(f"\nCreating {num_files} files in {fmt_name} format...")
            sidecar.set_default_format(fmt)
            
            total_size = 0
            for i in range(num_files):
                image_path = os.path.join(temp_dir, f"{fmt_name.lower()}_image_{i:03d}.jpg")
                with open(image_path, "wb") as f:
                    f.write(b"fake image data " * 50)
                
                data = {
                    "face_detection": {
                        "faces": [
                            {
                                "bbox": [i*10, i*10, 100, 100], 
                                "confidence": 0.8 + (i % 20) * 0.01,
                                "landmarks": [[i*10+5, i*10+5], [i*10+95, i*10+5], [i*10+50, i*10+50]],
                                "emotions": {"happy": 0.7 + (i % 10) * 0.02, "neutral": 0.3 - (i % 10) * 0.02}
                            }
                        ],
                        "processing_time": i * 0.01,
                        "model_version": "v1.2.3"
                    }
                }
                
                result = sidecar.create_sidecar(image_path, OperationType.FACE_DETECTION, data)
                total_size += os.path.getsize(result['sidecar_path'])
            
            total_sizes[fmt_name] = total_size
            print(f"Total size: {total_size:,} bytes ({total_size/1024:.1f} KB)")
        
        # Calculate efficiency
        json_total = total_sizes['JSON']
        binary_total = total_sizes['Binary'] 
        rkyv_total = total_sizes['Rkyv']
        
        print(f"\n📈 EFFICIENCY COMPARISON:")
        print(f"JSON total:   {json_total:,} bytes ({json_total/1024:.1f} KB)")
        print(f"Binary total: {binary_total:,} bytes ({binary_total/1024:.1f} KB)")
        print(f"Rkyv total:   {rkyv_total:,} bytes ({rkyv_total/1024:.1f} KB)")
        
        print(f"\nCompression efficiency:")
        print(f"Binary vs JSON: {json_total/binary_total:.2f}x smaller")
        print(f"Rkyv vs JSON:   {json_total/rkyv_total:.2f}x smaller")
        print(f"Rkyv vs Binary: {binary_total/rkyv_total:.2f}x smaller")
        
        print(f"\nSpace savings:")
        print(f"Binary saves: {((json_total-binary_total)/json_total)*100:.1f}%")
        print(f"Rkyv saves:   {((json_total-rkyv_total)/json_total)*100:.1f}%")


def show_format_characteristics():
    """Show the characteristics of each format."""
    print(f"\n📋 FORMAT CHARACTERISTICS")
    print("=" * 60)
    
    characteristics = [
        ("JSON", [
            "Human-readable text format",
            "Largest file sizes",
            "Easiest to debug and inspect",
            "Standard web format",
            "Good for development and testing"
        ]),
        ("Binary (Bincode)", [
            "Binary serialization format",
            "Medium file sizes",
            "Fast serialization/deserialization", 
            "Good balance of size and speed",
            "Cross-platform compatible"
        ]),
        ("Rkyv", [
            "Zero-copy binary format",
            "Smallest file sizes",
            "Fastest deserialization",
            "Memory-efficient",
            "Best for high-performance scenarios"
        ])
    ]
    
    for fmt_name, features in characteristics:
        print(f"\n{fmt_name}:")
        for feature in features:
            print(f"  • {feature}")


if __name__ == "__main__":
    compare_format_sizes()
    demonstrate_format_efficiency()
    show_format_characteristics()
    
    print(f"\n🎯 SUMMARY:")
    print("• Rkyv is typically the smallest format")
    print("• Binary provides good size/speed balance")
    print("• JSON is largest but most readable")
    print("• Format choice depends on use case:")
    print("  - Development: JSON")
    print("  - Production: Binary or Rkyv")
    print("  - High-performance: Rkyv")
