import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Product {
    id: number;
    name: string;
    price: number;
    description: string;
    category: string;
}

interface AfterLoginProps {
    userEmail: string | null;
    onLogout: () => Promise<void>;
}

const AfterLogin: React.FC<AfterLoginProps> = ({ userEmail, onLogout }) => {
    const navigate = useNavigate();
    const [products, setProducts] = useState<Product[]>([]);
    const [editingProduct, setEditingProduct] = useState<Product | null>(null);

    // Form state
    const [productName, setProductName] = useState("");
    const [productPrice, setProductPrice] = useState("");
    const [productDescription, setProductDescription] = useState("");
    const [productCategory, setProductCategory] = useState("");

    // Load products from SQLite via backend
    useEffect(() => {
        fetchProducts();
    }, []);

    const fetchProducts = async () => {
        try {
            const result: Product[] = await invoke("get_products");
            setProducts(result);
        } catch (err) {
            console.error("Failed to fetch products", err);
        }
    };

    const resetForm = () => {
        setProductName("");
        setProductPrice("");
        setProductDescription("");
        setProductCategory("");
        setEditingProduct(null);
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();

        if (!productName || !productPrice || !productDescription || !productCategory) {
            alert("Please fill in all fields");
            return;
        }

        try {
            if (editingProduct) {
                await invoke("update_product", {
                    id: editingProduct.id,
                    name: productName,
                    price: parseFloat(productPrice),
                    description: productDescription,
                    category: productCategory,
                });
            } else {
                await invoke("add_product", {
                    name: productName,
                    price: parseFloat(productPrice),
                    description: productDescription,
                    category: productCategory,
                });
            }

            await fetchProducts();
            resetForm();
        } catch (err) {
            console.error("Error saving product", err);
        }
    };

    const handleEdit = (product: Product) => {
        setEditingProduct(product);
        setProductName(product.name);
        setProductPrice(product.price.toString());
        setProductDescription(product.description);
        setProductCategory(product.category);
    };

    const handleDelete = async (id: number) => {
        if (window.confirm("Are you sure you want to delete this product?")) {
            try {
                await invoke("delete_product", { id });
                await fetchProducts();
            } catch (err) {
                console.error("Error deleting product", err);
            }
        }
    };

    const handleLogoutClick = async () => {
        await onLogout();
        navigate("/");
    };

    return (
        <div className="container">
            <h1>Welcome, {userEmail}!</h1>
            <h2>Product Management System</h2>

            {/* Product Form */}
            <div className="form-section">
                <h3>{editingProduct ? "Edit Product" : "Add New Product"}</h3>
                <form onSubmit={handleSubmit} className="product-form">
                    <div className="form-group">
                        <input
                            type="text"
                            placeholder="Product Name"
                            value={productName}
                            onChange={(e) => setProductName(e.target.value)}
                            className="form-input"
                        />
                    </div>

                    <div className="form-group">
                        <input
                            type="number"
                            placeholder="Price"
                            value={productPrice}
                            onChange={(e) => setProductPrice(e.target.value)}
                            className="form-input"
                            step="0.01"
                        />
                    </div>

                    <div className="form-group">
                        <textarea
                            placeholder="Description"
                            value={productDescription}
                            onChange={(e) => setProductDescription(e.target.value)}
                            className="form-input"
                            rows={3}
                        />
                    </div>

                    <div className="form-group">
                        <input
                            type="text"
                            placeholder="Category"
                            value={productCategory}
                            onChange={(e) => setProductCategory(e.target.value)}
                            className="form-input"
                        />
                    </div>

                    <div className="form-buttons">
                        <button type="submit" className="button primary">
                            {editingProduct ? "Update Product" : "Add Product"}
                        </button>
                        {editingProduct && (
                            <button
                                type="button"
                                onClick={resetForm}
                                className="button secondary"
                            >
                                Cancel
                            </button>
                        )}
                    </div>
                </form>
            </div>

            {/* Products List */}
            <div className="products-section">
                <h3>Your Products ({products.length})</h3>
                {products.length === 0 ? (
                    <p>No products found. Add your first product above!</p>
                ) : (
                    <div className="products-grid">
                        {products.map((product) => (
                            <div key={product.id} className="product-card">
                                <h4>{product.name}</h4>
                                <p className="product-price">${product.price.toFixed(2)}</p>
                                <p className="product-description">{product.description}</p>
                                <p className="product-category">Category: {product.category}</p>
                                <div className="product-actions">
                                    <button
                                        onClick={() => handleEdit(product)}
                                        className="button edit"
                                    >
                                        Edit
                                    </button>
                                    <button
                                        onClick={() => handleDelete(product.id)}
                                        className="button delete"
                                    >
                                        Delete
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {/* Logout Button */}
            <div className="logout-section">
                <button onClick={handleLogoutClick} className="button logout">
                    Logout
                </button>
            </div>
        </div>
    );
};

export default AfterLogin;
